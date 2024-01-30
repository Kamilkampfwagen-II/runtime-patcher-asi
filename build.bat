@echo off
cargo build --release --target=i686-pc-windows-msvc
copy /Y target\i686-pc-windows-msvc\release\runtime_patcher.dll target\i686-pc-windows-msvc\release\runtime_patcher.asi > nul
copy /Y target\i686-pc-windows-msvc\release\runtime_patcher.dll "%USERPROFILE%\Desktop\Chess Titans x86\runtime_patcher.asi" > nul
