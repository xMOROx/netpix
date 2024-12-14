import sys
import re

def extract_sdp_content(lines):
    sdp_lines = []
    in_sdp_section = False
    
    for line in lines:
        stripped = line.strip()
        
        if "Session Description Protocol" in stripped:
            in_sdp_section = True
            continue
            
        if in_sdp_section and stripped:
            # Extract the full SDP line including the type prefix
            if "Media Description" in stripped:
                prefix = "m="
            elif "Connection Information" in stripped:
                prefix = "c="
            elif "Bandwidth Information" in stripped:
                prefix = "b="
            elif "Media Attribute" in stripped:
                prefix = "a="
            else:
                continue

            parts = stripped.split(": ", 1)
            if len(parts) == 2:
                # Add the prefix to the content
                sdp_line = prefix + parts[1].strip()
                sdp_lines.append(sdp_line)

    final_lines = []
    first_media = True
    for line in sdp_lines:
        if line.startswith("m="):
            if not first_media:
                final_lines.append("")
            first_media = False
        final_lines.append(line)
    
    return final_lines

def main():
    if len(sys.argv) != 3:
        print("Usage: python parse_wireshark_sdp.py input.txt output.txt")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    try:
        with open(input_file, 'r') as f:
            input_lines = f.readlines()

        sdp_content = extract_sdp_content(input_lines)

        with open(output_file, 'w') as f:
            for line in sdp_content:
                f.write(line + '\n')
                
    except FileNotFoundError:
        print(f"Error: Could not find file {input_file}")
        sys.exit(1)
    except Exception as e:
        print(f"Error: {str(e)}")
        sys.exit(1)

if __name__ == "__main__":
    main()