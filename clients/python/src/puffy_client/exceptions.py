class PuffyError(Exception):
    pass


class PuffyTransportError(PuffyError):
    pass


class PuffyHTTPError(PuffyError):
    def __init__(self, status_code: int, body: str):
        super().__init__(f"HTTP {status_code}: {body}")
        self.status_code = status_code
        self.body = body


class PuffyAPIError(PuffyError):
    pass

