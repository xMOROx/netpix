package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"

	"labeler"

	"gopkg.in/yaml.v3"
)

type FileAnalysis struct {
	TotalFiles    int            `json:"total_files"`
	FilesByStatus map[string]int `json:"files_by_status"`
	FilesByExt    map[string]int `json:"files_by_extension"`
	ChangesByPath map[string]int `json:"changes_by_path"`
}

func analyzeFiles(files []any) FileAnalysis {
	analysis := FileAnalysis{
		TotalFiles:    len(files),
		FilesByStatus: make(map[string]int),
		FilesByExt:    make(map[string]int),
		ChangesByPath: make(map[string]int),
	}

	for _, f := range files {
		file := f.(map[string]any)
		filename := file["filename"].(string)
		status := file["status"].(string)

		analysis.FilesByStatus[status]++

		ext := filepath.Ext(filename)
		analysis.FilesByExt[ext]++

		dir := filepath.Dir(filename)
		analysis.ChangesByPath[dir]++
	}

	return analysis
}

func main() {
	prFile := flag.String("pr", "", "PR data JSON file")
	rulesFile := flag.String("rules", ".github/labeler.yml", "Rules YAML file")
	debug := flag.Bool("debug", false, "Enable debug output")
	flag.Parse()

	if *debug {
		log.SetFlags(log.Lshortfile | log.LstdFlags)
		log.Println("Debug mode enabled")
		log.Printf("Loading PR data from: %s", *prFile)
		log.Printf("Loading rules from: %s", *rulesFile)
	}

	registry := labeler.NewRuleRegistry()
	registry.Register("head-branch", &labeler.BranchRuleBuilder{})
	registry.Register("title", &labeler.TitleRuleBuilder{})
	registry.Register("body", &labeler.BodyRuleBuilder{})
	registry.Register("changed-files", &labeler.FilesRuleBuilder{})
	registry.Register("any", labeler.NewCompositeRuleBuilder(registry, false))
	registry.Register("all", labeler.NewCompositeRuleBuilder(registry, true))
	registry.Register("or", labeler.NewOrRuleBuilder(registry))

	prData, err := loadPRData(*prFile)
	if err != nil {
		log.Fatalf("Failed to load PR data: %v", err)
	}
	if *debug {
		log.Printf("Loaded PR data: title=%q, branch=%q, files=%d",
			prData.Title, prData.Branch, len(prData.ChangedFiles))
		log.Printf("PR Title: %q", prData.Title)
		log.Printf("PR Branch: %q", prData.Branch)
		log.Printf("PR Body: %q", prData.Body)
		log.Printf("Changed files: %v", prData.ChangedFiles)
	}

	rules, err := loadRules(*rulesFile)
	if err != nil {
		log.Fatalf("Failed to load rules: %v", err)
	}
	if *debug {
		log.Printf("Loaded %d rules", len(rules))
	}

	results := make(map[string]labeler.MatchResult)
	for label, ruleConfig := range rules {
		if *debug {
			log.Printf("Processing rule for label %q", label)
		}

		rule, err := parseRule(registry, ruleConfig)
		if err != nil {
			log.Printf("Failed to parse rule for label %s: %v", label, err)
			continue
		}

		result := rule.Evaluate(prData)
		if *debug {
			log.Printf("Rule evaluation for %q: matched=%v", label, result.Matched)
			switch debugInfo := result.Debug.(type) {
			case string:
				log.Printf("Debug info for %q: %s", label, debugInfo)
			case []string:
				log.Printf("Debug info for %q: %v", label, debugInfo)
			case map[string]interface{}:
				log.Printf("Debug info for %q: %+v", label, debugInfo)
			default:
				if result.Debug != nil {
					log.Printf("Debug info for %q: %v", label, result.Debug)
				}
			}
		}

		if result.Matched {
			results[label] = result
		}
	}

	if *debug {
		log.Printf("Matched labels: %v", getMatchedLabels(results))
	}

	output := map[string]any{
		"labels":    getMatchedLabels(results),
		"debugInfo": results,
	}

	if prData.Files != nil {
		fileAnalysis := analyzeFiles(prData.Files)
		output["file_analysis"] = fileAnalysis
		if *debug {
			log.Printf("File analysis: %+v", fileAnalysis)
		}
	}

	json.NewEncoder(os.Stdout).Encode(output)
}

func loadPRData(file string) (*labeler.PRContext, error) {
	data, err := ioutil.ReadFile(file)
	if err != nil {
		return nil, err
	}

	var prData labeler.PRContext
	if err := json.Unmarshal(data, &prData); err != nil {
		return nil, err
	}

	return &prData, nil
}

func loadRules(file string) (map[string]any, error) {
	data, err := ioutil.ReadFile(file)
	if err != nil {
		return nil, err
	}

	var rules map[string]any
	if err := yaml.Unmarshal(data, &rules); err != nil {
		return nil, err
	}

	return rules, nil
}

func parseRule(registry *labeler.RuleRegistry, config any) (labeler.Rule, error) {
	ruleList, ok := config.([]any)
	if !ok {
		return nil, fmt.Errorf("invalid rule configuration: expected array")
	}

	if len(ruleList) == 0 {
		return nil, fmt.Errorf("empty rule configuration")
	}

	ruleConfig, ok := ruleList[0].(map[string]any)
	if !ok {
		return nil, fmt.Errorf("invalid rule configuration: expected map")
	}

	for ruleType, _ := range ruleConfig {
		if ruleType == "any" || ruleType == "all" {
			return registry.Build(ruleType, ruleConfig)
		}
	}

	for ruleType, _ := range ruleConfig {
		return registry.Build(ruleType, map[string]any{ruleType: ruleConfig[ruleType]})
	}

	return nil, fmt.Errorf("no valid rule type found")
}

func getMatchedLabels(results map[string]labeler.MatchResult) []string {
	labels := make([]string, 0, len(results))
	for label := range results {
		labels = append(labels, label)
	}
	return labels
}
