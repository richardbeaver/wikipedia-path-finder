import { type FormEvent } from "react";

function UserGuessForm({
  onSubmit,
  startingURL,
  startingTitle,
  endingURL,
}: {
  onSubmit: (e: FormEvent<HTMLFormElement>) => void;
  startingURL: string;
  startingTitle: string;
  endingURL: string;
}) {
  return (
    <form className="d-grid gap-3" onSubmit={onSubmit}>
      <label htmlFor="user-guess">
        <h4>
          What's the minimum number of link clicks to get from the{" "}
          <a href={startingURL} target="_blank" rel="noopener noreferrer">
            {startingTitle}{" "}
            <i className="fa-solid fa-arrow-up-right-from-square"></i>
          </a>{" "}
          Wikipedia page to the{" "}
          <a href={endingURL} target="_blank" rel="noopener noreferrer">
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
  );
}

export default UserGuessForm;
