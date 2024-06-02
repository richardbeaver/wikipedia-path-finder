import os
import sys
from flask import Flask, abort
from flask_cors import CORS
from dotenv import load_dotenv
from wikipedia_crawler import WikipediaCrawler


load_dotenv()
VITE_BACKEND_PORT = os.getenv("VITE_BACKEND_PORT")
if VITE_BACKEND_PORT is None:
    print("No BACKEND_PORT environment variable in .env. Exiting...")
    sys.exit()


app = Flask(__name__)
CORS(app)


@app.route("/<starting_page>", methods=["GET"])
def crawl(starting_page: str):
    path = WikipediaCrawler(starting_page).crawl()
    result = {
        "starting_page": starting_page,
    }
    if path is None:
        result["result"] = ["Could not reach Kevin Bacon"]
    else:
        result["result"] = path

    return result


@app.route("/favicon.ico")
def handle_favicon():
    return abort(404)


if __name__ == "__main__":
    app.run(port=int(VITE_BACKEND_PORT))
