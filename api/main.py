import os
import sys
from flask import Flask, abort
from dotenv import load_dotenv
from wikipedia_crawler import WikipediaCrawler


load_dotenv()
BACKEND_PORT = os.getenv("BACKEND_PORT")
if BACKEND_PORT is None:
    print("No BACKEND_PORT environment variable in .env. Exiting...")
    sys.exit()


app = Flask(__name__)


@app.route("/<starting_page>", methods=["GET"])
def crawl(starting_page: str):
    min_hops = WikipediaCrawler(starting_page).get_min_hops()
    result = {
        "starting_page": starting_page,
    }
    if min_hops is None:
        result["result"] = "Could not reach Kevin Bacon"
    else:
        result["result"] = str(min_hops)

    return result


@app.route("/favicon.ico")
def handle_favicon():
    return abort(404)


if __name__ == "__main__":
    app.run(port=int(BACKEND_PORT))
