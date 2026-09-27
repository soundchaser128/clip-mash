import httpx
import time

SERVER = "http://localhost:5174"


def fetch_all_videos(client: httpx.Client):
    """
    Fetch all videos from the server.
    """
    page = 0
    videos = []
    while True:
        resp = client.get(
            "/api/library/video",
            params={
                "page": page,
                "size": 100,
            },
        )
        data = resp.json()
        if len(data["content"]) == 0:
            break
        videos.extend(data["content"])
        page += 1
        time.sleep(0.1)

    return videos


def create_marker(client: httpx.Client, video):
    payload = {
        "marker": {
            "videoId": video["id"],
            "start": 0.0,
            "end": video["duration"],
            "title": "Untitled",
            "indexWithinVideo": 0,
            "videoInteractive": False,
        },
        "createInStash": False,
    }

    return client.post(
        "/api/library/marker",
        json=payload,
    )


def main():
    client = httpx.Client(base_url=SERVER)
    all_videos = fetch_all_videos(client)
    print(f"Fetched {len(all_videos)} videos from the server.")
    for video in all_videos:
        markers = video["markerCount"]
        video = video["video"]
        if markers == 0:
            create_marker(client, video)
            print(f"Created marker for video {video['id']} - {video['title']}")


if __name__ == "__main__":
    main()
