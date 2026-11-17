# Deliberately vulnerable fixture for evaluation case 1. Never deploy.
import os

BASE = "/srv/files"


def download(name):
    with open(os.path.join(BASE, name), "rb") as f:
        return f.read()
