use clap::Parser;

#[derive(Parser)]
#[clap(name = "file-local-ip-replacer")]
#[clap(author = "Jtplouffe")]
#[clap(version = "0.1")]
#[clap(about = "Replace IPs in a file with the local network IP of the host machine")]
pub(crate) struct CliArgs {
    #[clap(help = "Path of the file to replace the IPs")]
    pub(crate) path: String,

    #[clap(long, help = "Also replace 'localhost'")]
    pub(crate) replace_localhost: bool,
}
