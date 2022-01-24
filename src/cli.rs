use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Hello world",
  about = "Hello world"
)]
pub struct CommandLineArgs {
}
