use clap::ArgMatches;

pub fn extract_command_argument<T>(
    command_arguments: &ArgMatches,
    field_name: &'static str,
) -> Option<T>
where
    T: Clone + Sync + Send + 'static,
{
    match command_arguments.get_one::<T>(field_name) {
        Some(value) => Some(value.to_owned()),
        None => None,
    }
}
