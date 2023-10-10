function SearchResults({
  userGuess,
  path,
  timedOut,
}: {
  userGuess: number | undefined;
  path: string[];
  timedOut: boolean;
}) {
  return (
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
  );
}

export default SearchResults;
