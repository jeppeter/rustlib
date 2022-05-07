#[allow(dead_code)]
#[structopt(name = "rename_all", rename_all = "screaming_snake_case")] 
enum Opt
{
    /// A screaming loud first command. Only use if necessary.
    FirstCommand
    {
                    /// This flag will even scream louder.
                    #[structopt(long, short)] foo : bool,
    }, /// Not nearly as loud as the first command.
    #[structopt(rename_all = "pascal_case")] 
    SecondCommand
    {
        /// Nice quiet flag. No one is annoyed.
        #[structopt(rename_all = "snake_case", long)] bar_option : bool,
        #[structopt(subcommand)] cmds : Subcommands, #[structopt(flatten)]
        options : BonusOptions,
    },
}


#[allow(unknown_lints)]
#[allow(unused_variables, dead_code, unreachable_code)]
#[allow(clippy :: style, clippy :: complexity, clippy :: pedantic, clippy ::
	restriction, clippy :: perf, clippy :: deprecated, clippy :: nursery,
	clippy :: cargo)] #[deny(clippy :: correctness)] 
impl ::structopt::StructOpt for Opt
{
	fn clap<'a,'b>() -> ::structopt::clap::App<'a,'b>
	{
		let app = ::structopt::clap::App::new("rename_all").setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp) ;
		< Self as ::structopt::StructOptInternal>::augment_clap(app)
	} 
	fn from_clap(matches : &::structopt::clap::ArgMatches) -> Self
	{
		< Self as ::structopt::StructOptInternal>::from_subcommand(matches.subcommand()).expect("structopt misuse: You likely tried to #[flatten] a struct \
			that contains #[subcommand]. This is forbidden.")
	}
} 
#[allow(unused_variables)] #[allow(unknown_lints)]
#[allow(clippy::style, clippy::complexity, clippy::pedantic, 
	clippy::restriction, clippy::perf, clippy::deprecated, clippy::nursery,
	clippy::cargo)] 
#[deny(clippy::correctness)]
#[allow(dead_code, unreachable_code)] 
impl ::structopt::StructOptInternal for Opt
{
	fn augment_clap<'a,'b> (app : ::structopt::clap::App<'a,'b>)
	-> ::structopt::clap::App<'a,'b>
	{
		let app = app ; 
		let app = app.subcommand({
			let subcommand = ::structopt::clap::SubCommand::with_name("FIRST_COMMAND") ;
			{
				let subcommand =   subcommand.about("A screaming loud first command. Only use if necessary");
				let subcommand =   subcommand.arg(
					::structopt::clap::Arg::with_name("FOO").takes_value(false).multiple(false).help("This flag will even scream louder").long("FOO").short("FOO"));
				subcommand.version("0.1.0")
			}
		});
		let app = app.subcommand({
			let subcommand = ::structopt::clap::SubCommand::with_name("SecondCommand");
			{
				let subcommand = subcommand.about("Not nearly as loud as the first command");
				let subcommand = subcommand.arg(::structopt::clap::Arg::with_name("bar_option").takes_value(false).multiple(false).help("Nice quiet flag. No one is annoyed").long("bar_option"));
				let subcommand = <BonusOptions as ::structopt::StructOptInternal>::augment_clap(subcommand); 
				let subcommand = if  <BonusOptions as ::structopt::StructOptInternal>::is_subcommand()
				{
					subcommand.setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
				} else { 
					subcommand 
				}; 
				let subcommand = <Subcommands as ::structopt::StructOptInternal>::augment_clap(subcommand); 
				let subcommand = subcommand.setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp); 
				subcommand.version("0.1.0")
			}
		});; 
		app.version("0.1.0")
	} 
	fn from_subcommand<'a,'b>
	(sub :(&'b str, Option <&'b ::structopt::clap::ArgMatches<'a>>)) ->   Option <Self>
	{
		match sub
		{
			("FIRST_COMMAND", Some(matches)) =>
			{ Some(Opt::FirstCommand { 
				foo : matches.is_present("FOO") 
			}) },
			("SecondCommand", Some(matches)) =>
			{
				Some(Opt::SecondCommand
				{
					bar_option : matches.is_present("bar_option"), 
					cmds :  <Subcommands as ::structopt::StructOptInternal>::from_subcommand(matches.subcommand()).unwrap(),
					options : ::structopt::StructOpt::from_clap(matches)
				})
			}, 
			other => {  None }
		}
	} 
	fn is_subcommand() -> bool {
		true 
	}
}


enum Subcommands { FirstSubcommand, }

#[allow(unknown_lints)]
#[allow(unused_variables, dead_code, unreachable_code)]
#[allow(clippy::style, clippy::complexity, clippy::pedantic, clippy::restriction, clippy::perf, clippy::deprecated, clippy::nursery, clippy::cargo)] 
#[deny(clippy::correctness)] 
impl ::structopt::StructOpt for Subcommands
{
	fn clap<'a, 'b > () -> :: structopt :: clap :: App < 'a, 'b >
	{
		let app = :: structopt :: clap :: App ::
		new("tststropt").setting(:: structopt :: clap :: AppSettings ::
			SubcommandRequiredElseHelp) ; < Self as ::
		structopt :: StructOptInternal > :: augment_clap(app)
	} fn from_clap(matches : & :: structopt :: clap :: ArgMatches) -> Self
	{
		< Self as :: structopt :: StructOptInternal > ::
		from_subcommand(matches.subcommand()).expect("structopt misuse: You likely tried to #[flatten] a struct \
			that contains #[subcommand]. This is forbidden.")
	}
} 
#[allow(unused_variables)] #[allow(unknown_lints)]
#[allow(clippy :: style, clippy :: complexity, clippy :: pedantic, clippy ::
	restriction, clippy :: perf, clippy :: deprecated, clippy :: nursery,
	clippy :: cargo)] #[deny(clippy :: correctness)]
#[allow(dead_code, unreachable_code)] impl :: structopt :: StructOptInternal
for Subcommands
{
	fn augment_clap < 'a, 'b > (app : :: structopt :: clap :: App < 'a, 'b >)
	-> :: structopt :: clap :: App < 'a, 'b >
	{
		let app = app ;
		let app = app.subcommand({
			let subcommand = :: structopt :: clap :: SubCommand
			:: with_name("first-subcommand") ;
			subcommand.version("0.1.0")
		}) ; 
		app.version("0.1.0")
	} 
	fn from_subcommand<'a,'b>(sub :(&'b str, Option<&'b ::structopt::clap::ArgMatches <'a >>)) -> Option <Self>
	{
		match sub
		{
			("first-subcommand", Some(matches)) =>
			{ Some(Subcommands::FirstSubcommand) }, 
			other => {  None }
		}
	} 
	fn is_subcommand() -> bool { true }
}

#[allow(dead_code)] 
struct BonusOptions
{ 
	#[structopt(long)]
	baz_option : bool, 
}


#[allow(unused_variables)] #[allow(unknown_lints)]
#[allow(clippy :: style, clippy :: complexity, clippy :: pedantic, clippy ::
	restriction, clippy :: perf, clippy :: deprecated, clippy :: nursery,
	clippy :: cargo)] #[deny(clippy :: correctness)]
#[allow(dead_code, unreachable_code)] impl ::structopt::StructOpt for BonusOptions
{
	fn clap < 'a, 'b > () -> :: structopt :: clap :: App < 'a, 'b >
	{
		let app = :: structopt :: clap :: App :: new("tststropt") ; < Self as
		:: structopt :: StructOptInternal > :: augment_clap(app)
	} 
	fn from_clap(matches : &::structopt::clap::ArgMatches) -> Self
	{ 
		BonusOptions { baz_option : matches.is_present("baz-option") 
	} 
}
} 
#[allow(unused_variables)] #[allow(unknown_lints)]
#[allow(clippy :: style, clippy :: complexity, clippy :: pedantic, clippy ::
	restriction, clippy :: perf, clippy :: deprecated, clippy :: nursery,
	clippy :: cargo)] #[deny(clippy :: correctness)]
#[allow(dead_code, unreachable_code)] impl ::structopt::StructOptInternal for BonusOptions
{
	fn augment_clap < 'a, 'b > (app : :: structopt :: clap :: App < 'a, 'b >) -> ::structopt::clap::App < 'a, 'b >
	{
		{
			let app = app ; let app =
			app.arg(:: structopt :: clap :: Arg ::
				with_name("baz-option").takes_value(false).multiple(false).long("baz-option"))
			; app.version("0.1.0")
		}
	} fn is_subcommand() -> bool { false }
}