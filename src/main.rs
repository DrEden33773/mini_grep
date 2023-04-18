use mini_grep::Config;
use std::{env, process};

type SuperError = Box<dyn std::error::Error>;

pub fn main() -> Result<(), SuperError> {
    let config = Config::build_from_iter(env::args_os()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(-1);
    });
    if let Err(err) = config.run() {
        eprintln!("Application error: {err}");
        process::exit(0);
    }
    Ok(())
}

#[cfg(test)]
mod preludes {
    use std::{
        fs::{self, File},
        io::Write,
    };

    #[test]
    fn create_dir_and_file() -> std::io::Result<()> {
        let contents = "
I'm nobody! Who are you?
我啥也不是，你呢？
Are you nobody, too?
牛逼如你也是无名之辈吗？
Then there's a pair of us - don't tell!
那我们就是天生一对，嘘！别说话！
They'd banish us, you know.
你知道，我们不属于这里。
How dreary to be somebody!
因为这里属于没劲的大人物！
How public, like a frog
他们就像青蛙一样呱噪，
To tell your name the livelong day
成天将自己的大名
To an admiring bog!
传遍整个无聊的沼泽！
";
        fs::create_dir_all("test_files")?;
        let mut file = File::create("test_files/poem.txt")?;
        file.write_fmt(format_args!("{}", contents))?;
        Ok(())
    }
}
