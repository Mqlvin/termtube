# TermTube
An application for listening to YouTube audio in the terminal, utilising `yt-dlp` and `ffplay`.

## Building
Install `cargo` from your favourite package manager.

Clone the repo with `git clone git@github.com:Mqlvin/termtube.git`
Move into the cloned directory `cd termtube`
Build with cargo `cargo build --release`

A static binary will be produced at `target/release/termtube`
You can move this binary onto your system PATH to use it anywhere :)


## Usage and Running
*Prerequisite: Ensure you have `yt-dlp` and `ffplay` installed and available on your system PATH.*

To get started, type `termtube --help`

#### Basic Usage
A source can be a YouTube video link, a YouTube playlist link, or a source file.
To play YouTube links, type `termtube -s <source> -s <source> -s <source>...`

#### Using files as a source
You can compile a list of YouTube video links and YouTube playlist links into a file to be supplied to TermTube.
Ensure each link is placed on a new line.
*Note: Adding lots of playlists can significantly increase startup time of TermTube. This is due to extra requests being made to YouTube.*

#### Other useful options
For looping, add the `--loop` argument.
For shuffling, add the `--shuffle` argument.


## Licensing
The project is licensed under the MIT license, which can be found [here](https://github.com/Mqlvin/termtube/blob/master/LICENSE).
