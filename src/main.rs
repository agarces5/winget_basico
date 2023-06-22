mod command;
mod command_builder;

use command::WingetCommand;
use command_builder::Builder;

fn main() {
    WingetCommand::builder()
        .with_id("7zip.7zip")
        .build()
        .execute();
    WingetCommand::builder()
        .with_id("Adobe.Acrobat.Reader.64-bit")
        .build()
        .execute();
    WingetCommand::builder()
        .with_id("Oracle.JavaRuntimeEnvironment")
        .build()
        .execute();
    WingetCommand::builder()
        .with_id("TheDocumentFoundation.LibreOffice")
        .build()
        .execute();
    WingetCommand::builder()
        .with_id("RealVNC.VNCServer")
        .with_version("6.11.0.47988")
        .build()
        .execute();
    WingetCommand::builder()
        .with_id("RealVNC.VNCViewer")
        .build()
        .execute();
    WingetCommand::builder()
        .with_id("Google.Chrome")
        .build()
        .execute();
    WingetCommand::builder()
        .with_id("RARLab.WinRAR")
        .build()
        .execute();
}
