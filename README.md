# TermTube
An application for listening to YouTube audio in the terminal, utilising `yt-dlp` and `ffplay`.

<br>

## Building 🛠️
Install `cargo` from your favourite package manager.

Clone the repo with `git clone git@github.com:Mqlvin/termtube.git`<br>
Move into the cloned directory `cd termtube`<br>
Build with cargo `cargo build --release`<br>

A static binary will be produced at `./target/release/termtube`<br>
You can move this binary on to your system PATH to use it anywhere :)

<br>

## Usage and Running 🎶
*Prerequisite: Ensure you have `yt-dlp` and `ffplay` installed and available on your system PATH.*

To get started, type `termtube --help`

#### Basic Usage
To play audio, type `termtube --source <source> --source <source> --source <source>...`<br>
A source can be a YouTube video link, a YouTube playlist link, or a source file.<br>
As demonstrated, the `--source` flag (aka `-s`) can be used many times.

#### Using files as a source
You can create a list of YouTube video/playlist links in a file to be supplied to TermTube.<br>
Ensure each link is placed on a new line, or the program will not read the file correctly.<br>
*Note: Adding lots of playlists can significantly increase startup time of TermTube. This is due to extra requests being made to YouTube.*<br>

#### Other useful options
For looping, add the `--loop` argument.<br>
For shuffling, add the `--shuffle` argument.

<br>

## Licensing 📄
The project is licensed under the MIT license, which can be found [here](https://github.com/Mqlvin/termtube/blob/master/LICENSE).
