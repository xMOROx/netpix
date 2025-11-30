use std::panic;
use std::path::Path;

#[test]
fn parse_pcaps_without_panics() {
    let pcap_dir = Path::new("../pcap_examples");
    let entries = std::fs::read_dir(pcap_dir).expect("failed to read pcap_examples");

    let mut found = false;
    for entry in entries {
        let entry = entry.expect("failed to read dir entry");
        let path = entry.path();
        let ext = match path.extension().and_then(|s| s.to_str()) {
            Some(e) => e.to_ascii_lowercase(),
            None => continue,
        };
        if ext != "pcap" && ext != "pcapng" {
            continue;
        }

        found = true;
        let mut cap = pcap::Capture::from_file(&path).expect("failed to open pcap file");

        let mut i = 0usize;
        loop {
            let pkt = match cap.next_packet() {
                Err(pcap::Error::NoMorePackets) => break,
                Err(e) => panic!("Error reading pcap {:?}: {:?}", path, e),
                Ok(p) => p,
            };

            let res = panic::catch_unwind(|| {
                let maybe_packet = netpix_common::packet::Packet::build(&pkt, i);
                if let Some(mut packet) = maybe_packet {
                    packet.guess_payload();
                }
            });

            if res.is_err() {
                panic!("Panic while parsing {:?} at packet #{}", path, i);
            }

            i += 1;
        }
    }

    assert!(found, "No pcap files found in ../pcap_examples");
}
