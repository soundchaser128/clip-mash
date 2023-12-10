import {SongDto} from "@/api"
import ExternalLink from "@/components/ExternalLink"
import {formatSeconds} from "@/helpers/time"

function calcBPM(song: SongDto): string {
  return ((song.beats.length / song.duration) * 60.0).toFixed(0)
}

interface SongsTableProps {
  songs: SongDto[]
  selection: number[]
  onToggleSong: (songId: number, checked: boolean) => void
}

const SongsTable: React.FC<SongsTableProps> = ({
  songs,
  selection,
  onToggleSong,
}) => {
  return (
    <div className="overflow-x-auto flex flex-col w-2/3">
      <table className="table table-compact w-full">
        <thead>
          <tr>
            <th>Name</th>
            <th>Duration</th>
            <th>URL</th>
            <th>Beats per minute</th>
            <th>Include</th>
          </tr>
        </thead>
        <tbody>
          {songs.length === 0 && (
            <tr>
              <td className="text-center p-4" colSpan={5}>
                No music yet.
              </td>
            </tr>
          )}
          {songs.map((song) => (
            <tr key={song.songId}>
              <td>{song.fileName}</td>
              <td>{formatSeconds(song.duration, "short")}</td>
              <td>
                <ExternalLink href={song.url}>{song.url}</ExternalLink>
              </td>
              <td>{calcBPM(song)}</td>
              <td>
                <input
                  type="checkbox"
                  className="checkbox checkbox-primary"
                  checked={selection.includes(song.songId)}
                  onChange={(e) => onToggleSong(song.songId, e.target.checked)}
                />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

export default SongsTable
