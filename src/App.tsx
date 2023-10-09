import { useState, useEffect } from "react";
import axios from "axios";
import "./App.css";

const TIMEOUT_DURATION_MS = 8000;

const STARTING_PAGES = [
  // Pages that do get there in 8 seconds
  "Titanic_(1997_film)",
  "Some_Like_It_Hot",
  "Brokeback_Mountain",
  "The_Lord_of_the_Rings:_The_Fellowship_of_the_Ring",
  "The_Lord_of_the_Rings:_The_Return_of_the_King",
  "Superman_(1978_film)",
  "This_Is_Spinal_Tap",
  "Castle_in_the_Sky",
  "The_Dark_Knight",
  "Die_Hard",
  // Pages that don't
  "West_Side_Story_(1961_film)",
  "Hoop_Dreams",
  "Casablanca_(film)",
  "Bowling_for_Columbine",
  "Blazing_Saddles",
  "Monty_Python's_Life_of_Brian",
  "Rocky",
  "Pinocchio_(1940_film)",
  "Back_to_the_Future",
  "The_Rocky_Horror_Picture_Show",
  "The_Exorcist",
];

function App() {
  const BASE_URL = "https://en.wikipedia.org/wiki/";
  const KEVIN_BACON_TITLE = "Kevin_Bacon";
  const GET_RANDOM_TITLE_URL =
    "https://en.wikipedia.org/api/rest_v1/page/random/title";
  const SERVERLESS_FUNCTION_PATH = "/.netlify/functions/path-finder";

  const [path, setPath] = useState<string[]>([]);
  const [startingTitle, setStartingTitle] = useState("");
  const [startingURL, setStartingURL] = useState("");
  const [userGuess, setUserGuess] = useState(undefined);
  const [timedOut, setTimedOut] = useState(false);

  useEffect(() => {
    // setRandomWikipediaTitle();
    setTitleFromFilmList();
  }, []);

  function setTitleFromFilmList() {
    const idx = Math.floor(Math.random() * STARTING_PAGES.length);
    const title = STARTING_PAGES[idx];
    setStartingTitle(title);
    setStartingURL(BASE_URL + title);
  }

  function setRandomWikipediaTitle() {
    fetch(GET_RANDOM_TITLE_URL)
      .then((response) => response.json())
      .then((data) => {
        const title: string = data.items[0].title;
        setStartingTitle(title);
        setStartingURL(BASE_URL + title);
      })
      .catch((e) => console.error(e));
  }

  function findPath() {
    axios
      .get(`${SERVERLESS_FUNCTION_PATH}/${startingTitle}`, {
        timeout: TIMEOUT_DURATION_MS,
      })
      .then((response) => {
        setPath(response.data.path);
      })
      .catch((_e) => {
        setTimedOut(true);
      });
  }

  function handleSubmitGuess(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setUserGuess(e.target.userGuess.value);
    findPath();
  }

  return (
    <div className="App container-fluid p-3">
      <div className="row">
        <div className="col-5">
          <form className="d-grid gap-3" onSubmit={handleSubmitGuess}>
            <label htmlFor="user-guess">
              <h4>
                What's the minimum number of link clicks to get from the{" "}
                <a href={startingURL} target="_blank" rel="noopener noreferrer">
                  {startingTitle}{" "}
                  <i className="fa-solid fa-arrow-up-right-from-square"></i>
                </a>{" "}
                Wikipedia page to the{" "}
                <a
                  href={BASE_URL + KEVIN_BACON_TITLE}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  Kevin Bacon
                  <i className="fa-solid fa-arrow-up-right-from-square"></i>
                </a>{" "}
                Wikipedia page?
              </h4>
            </label>

            <input
              type="number"
              name="userGuess"
              className="form-control"
              id="user-guess"
              required
            />
            <button type="submit" className="btn btn-primary">
              Submit Guess
            </button>
          </form>

          <div className="py-3 px-1">
            {userGuess && <p>You guessed: {userGuess}</p>}

            {userGuess && path.length === 0 && <p>Searching...</p>}
            {timedOut && <p>Timed out before finishing search.</p>}

            {path.length > 0 && (
              <div>
                <h4>Answer:</h4>
                <p>{`Number of link hops: ${path.length - 1}`}</p>
                <span>{"Shortest path to get there: "}</span>
                {path.map((link: string) => (
                  <li>{link}</li>
                ))}
              </div>
            )}

            {(timedOut || path.length > 0) && (
              <button
                onClick={() => window.location.reload()}
                className="btn btn-primary m-3"
              >
                Try another one
              </button>
            )}
          </div>
        </div>
        <div className="col">
          <div className="container-fluid">
            <iframe id="wikipedia-iframe" src={startingURL} />
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
