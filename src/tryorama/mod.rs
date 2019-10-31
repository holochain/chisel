pub mod demux;

#[derive(Debug, StructOpt)]
pub enum TryoramaCmd {
  Demux(demux::DemuxCmd),
}
