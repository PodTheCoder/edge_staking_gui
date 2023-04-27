# Pod's Edge Staking GUI
In beta. Easy Edge Host Setup.

<img width="599" alt="Edge-Staking-Gui-v-0-5-0 - Copy" src="https://user-images.githubusercontent.com/113918164/234863196-63f22076-6c0c-45fc-aea5-450de2e08fb9.PNG">

## Features
- Easily setup your Edge host with a GUI (Graphical User Interface)
- Uses the *device token* staking method. Has no access to your XE wallet. 
- Runs on Windows. (Mac and Linux planned)

## Planned Features Ver 1.0
- Check if your device is online.
- Auto-launch application when your system starts.
- Minimize application to tray.

## Nerd Info
- Tauri framework for Cross-OS Desktop App.
- Frontend -> Vue with Vite.
- Backend -> Rust.
- Creates correct Edge CLI URL based on requirements & checks checksum.

## Development

Prerequisites: https://tauri.app/v1/guides/getting-started/prerequisites

Development & build commands can be found in the Makefile.

## Troubleshooting


Logs can be found in:

Windows = C:\Users\{YourUserName}\AppData\Local\EdgeStakingGUI

MacOS: Resolves to $HOME/Library/Application Support.

Linux = $XDG_DATA_HOME or $HOME/.local/share.

On Linux, you need the libraries:
OpenSSL 1.0.1, 1.0.2, 1.1.0, or 1.1.1 with headers (see https://github.com/sfackler/rust-openssl)

## Credits
Logo Photo by <a href="https://unsplash.com/@ortodummie?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Peter Žagar</a> on <a href="https://unsplash.com/photos/bLgWa9b0ioY?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>
  
## Disclaimer
THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
