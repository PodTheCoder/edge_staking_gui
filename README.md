# Pod's Edge Staking GUI (beta)

## Features
- Easily setup & control your Edge host with a GUI (Graphical User Interface).
- Uses the *device token* staking method. Has no access to your XE wallet.
- Autolaunch your node. Can also launch minimized for a seamless experience.
- Get notified when you receive node earnings or when your node is offline.
- Cross-platform 64-bit (Windows, Mac, and Linux.)
- Helpful error messages.
- Fast & small program.

## Screenshot
<img width="597" alt="edge-staking-gui - Copy" src="https://github.com/PodTheCoder/edge_staking_gui/assets/113918164/26217e9c-894d-47dd-8d4d-fcc30fd3d8f4">

## Frequently Asked Questions
Q: Does the GUI have access to my wallet?

A: No. The GUI is designed to use the *device code* staking method.

Q: How can I switch networks?

A: Click "Network" in the bottom-right of the GUI.

Q: When running the .msi installer I get the error: Microsoft Defender SmartScreen prevented an unrecognised app from starting.

A: This is expected because the Edge Staking GUI is a new program. To continue, click on "More info -> Run anyway"

Q: Help I am stuck. Where can I get support?

A: On the Edge Discord: https://ed.ge/discord. Ask in the *staking-support* channel.


## Nerd Info
- Tauri framework for Cross-OS Desktop App.
- Frontend -> Vue with Vite.
- Backend -> Rust.

## Development

Prerequisites: https://tauri.app/v1/guides/getting-started/prerequisites

Development & build commands can be found in the Makefile.

You can switch your network by clicking on "Network" in the bottom-right of the GUI.

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
