import unittest

from puffy_client import HealthResponse, JobInfo, JobResponse


class ModelTests(unittest.TestCase):
    def test_job_response(self):
        response = JobResponse(status="success", job=JobInfo(job_id="job-1", status="queued"))
        self.assertEqual(response.job.job_id, "job-1")

    def test_health_response(self):
        response = HealthResponse(status="ok", name="Puffy", version="0.1.0")
        self.assertEqual(response.name, "Puffy")


if __name__ == "__main__":
    unittest.main()

