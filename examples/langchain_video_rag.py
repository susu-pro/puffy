from puffy_client import PuffyClient


def main() -> None:
    client = PuffyClient()
    accepted = client.extract_async("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
    print(f"queued: {accepted.job_id}")
    print("hook this job into LangChain after the public engine returns a result")


if __name__ == "__main__":
    main()

