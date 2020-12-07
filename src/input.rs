// const SESSION_TOKEN: &str = env!("AOC_TOKEN");
const SESSION_TOKEN: &str = "53616c7465645f5f38d5352b28e104dc28f094f23bca3b7b860f9ae01fa77336df4460c43250fbfc31b0ed3071dc6d26";

type SurfError = Box<dyn std::error::Error + Send + Sync>;
type SurfResult<T> = Result<T, SurfError>;
#[derive(Debug)]
pub enum InputError {
    Network(SurfError),
    Io(async_std::io::Error),
}

impl From<SurfError> for InputError {
    fn from(err: SurfError) -> Self {
        Self::Network(err)
    }
}

impl From<async_std::io::Error> for InputError {
    fn from(err: async_std::io::Error) -> Self {
        Self::Io(err)
    }
}

pub async fn get_input(year: u32, day: u32) -> Result<String, InputError> {
    let path = format!("inputs/{}/{:0>2}.txt", year, day);
    match read_input(&path).await {
        Ok(input) => Ok(input),
        Err(_) => {
            let mut input = download_input(year, day).await?;
            if input.chars().last() == Some('\n') {
                input.truncate(input.len() - 1); // remove the trailing newline
            }
            write_input(year, path, &input).await?;
            Ok(input)
        }
    }
}

async fn download_input(year: u32, day: u32) -> SurfResult<String> {
    surf::get(format!(
        "https://adventofcode.com/{}/day/{}/input",
        year, day
    ))
    .set_header("cookie", format!("session={}", SESSION_TOKEN))
    .recv_string()
    .await
}

async fn read_input(path: &str) -> async_std::io::Result<String> {
    Ok(String::from_utf8(async_std::fs::read(path).await?).unwrap())
}

async fn write_input(year: u32, path: String, input: &str) -> async_std::io::Result<()> {
    async_std::fs::create_dir_all(format!("inputs/{}", year)).await?;
    async_std::fs::write(path, input).await
}

#[async_std::test]
async fn test_input() {
    match get_input(2018, 6).await {
        Ok(input) => println!("{}", input),
        Err(err) => {
            println!("{:?}", err);
            panic!()
        }
    }
}
