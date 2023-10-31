# Pod's Edge Staking GUI (beta)

## Features
- Easily setup & control your Edge host with a GUI (Graphical User Interface).
- Improve your security. By design, the GUI has no access to your XE wallet.
- Autolaunch your node. Can also launch minimized for a seamless experience.
- Get notified when you receive node earnings or when your node is offline.
- Cross-platform 64-bit for processors supported by the Edge CLI (Intel64/AMD64). Windows 7+, Mac 10.15+, and experimental Linux support.
- Helpful error messages.
- Fast & small program.

## Screenshot
<img width="597" alt="edge-staking-gui - Copy" src="https://github.com/PodTheCoder/edge_staking_gui/assets/113918164/26217e9c-894d-47dd-8d4d-fcc30fd3d8f4">

## Frequently Asked Questions
Q: How does the GUI improve security?

A: The GUI improves security compared to the traditional approach by implementing the *device token* staking method. The major benefit of this method is that the GUI has no access to your wallet. Furthermore, the official Edge software is used under the hood and the software is open source.

Q: How can I switch networks?

A: Network switching is an advanced feature. 

- If a stake is already setup, first click the button `Back To Setup.` 

- Stop the program (make sure it does not run minimized). 

- Then change your config.txt to: `network = 'testnet'` or `network = 'mainnet'`

- Restart the program & follow the steps in the GUI.

Q: My installer says the app/developer is unrecognized. Eg. Running the .msi installer I get the error: Microsoft Defender SmartScreen prevented an unrecognised app from starting.

A: This is expected because the Edge Staking GUI is a new program and does not yet have code signing. To continue on Microsoft Defender Smartscreen, click on "More info -> Run anyway". On MacOS you might also need to explicitly choose to trust the unidentified developer.

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

On Linux, depending on your distribution, you need several libraries:
- Reqwest requirements: https://github.com/seanmonstar/reqwest#requirements
- Tauri requirements: https://github.com/tauri-apps/tauri/blob/dev/README.md#get-started

## Credits
Logo Photo by <a href="https://unsplash.com/@ortodummie?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Peter Žagar</a> on <a href="https://unsplash.com/photos/bLgWa9b0ioY?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>
  
## Disclaimer
THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
