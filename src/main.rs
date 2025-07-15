// Filter a HMMER tblout file by E-value threshold

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // get the command line args, only parse the
    // first one which should be a fasta file
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        println!("Usage: filter_tblout <tblout_file> <E-value>");
        std::process::exit(1);
    }

    let mut reader = hmm_tblout::Reader::from_path(args[1].clone())?;
    let evalue = args[2].parse::<f32>()?;

    let header = reader.header().clone();
    let meta = reader.meta().clone();
    let records = reader.records();

    let mut writer = hmm_tblout::Writer::new(std::io::stdout());

    writer.write_header(header)?;
    for record in records {
        let r = record?;
        // I care about the full e-value for hmmscan
        // OR the regular e-value for cmscan
        let eval = match r.e_value_full() {
            Some(e) => e,
            None => r.e_value().unwrap(),
        };

        if eval < evalue {
            writer.write_record(&r)?;
        }
    }

    writer.write_meta(meta)?;

    Ok(())
}
