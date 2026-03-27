import unittest

from puffy_client import PuffyClient


class ClientTests(unittest.TestCase):
    def test_base_url_trim(self):
        client = PuffyClient(base_url="http://127.0.0.1:41480/")
        self.assertEqual(client.base_url, "http://127.0.0.1:41480")

    def test_build_url(self):
        client = PuffyClient()
        self.assertEqual(
            client._build_url("/api/search", {"q": "hello world"}),
            "http://127.0.0.1:41480/api/search?q=hello+world",
        )


if __name__ == "__main__":
    unittest.main()

