# Unofficial Edge Staking GUI

In alpha. Step 1 and 2 are tested on Windows 10. The other remaining steps are still in development.

<img width="594" alt="Edge_Staking_Gui_Ver_0_1_0" src="https://user-images.githubusercontent.com/113918164/232255569-e3b9fb54-fe19-478c-9b39-3507dcfc0bdc.PNG">

## Design Goals
- Easy to use.
- Minimal configuration.
- No administrator permissions required.
- Run on Windows, Mac or Linux. Initial focus on Windows.
- Don’t integrate wallet functionality into the GUI. The web Edge wallet already does that well. All a node needs is a stake. When correctly configured, version 1 of the Edge Staking GUI aims to have NO access to the wallet where your XE is held.

## Implemented Features
- Check if your system is ready for staking.
- Automatically intall the correct Edge CLI for your system.
- Start & Stop staking.
- Display current status & log for troubleshooting.

## Planned Features Ver 1.0
- Set up staking using only your device code.
- Check if your device is online.

## Planned Features Ver 1.5
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

## Disclaimer
THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
