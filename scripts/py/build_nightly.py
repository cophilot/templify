import datetime
import os


tag = f"nightly-{datetime.datetime.now().strftime('%Y-%m-%d')}"

os.system("cargo build --release")
