# Where's Kevin? - A Wikipedia Path Finder

- Web-hosted game that displays a randomly chosen Wikipedia article and asks user
  to guess minimum number of link hops to reach Kevin Bacon’s Wikipedia page
  (inspired by the [Six Degrees of Kevin Bacon](https://en.wikipedia.org/wiki/Six_Degrees_of_Kevin_Bacon)).

- Uses Breadth-First Search to find the shortest path from the starting page to Kevin Bacon's Wikipedia page.

The backend path-finding code is set up as a Netlify serverless function. Unfortunately, requests that take longer than ten seconds would need background functions, which are not available on Netlify's free tier.

Instead, I have placed a handful of films' Wikipedia page titles in an array in the backend code file, and choose one of those as the starting point, rather than a completely random Wikipedia page.

About half of the film pages in that list do reach Kevin Bacon's page before the timeout is hit, and the other half do not. This allowed me to get practice displaying page components that show when the result is found and when the timeout is reached instead.

The user experience could be made more efficient by starting the path-finding when the page is loaded, rather than waiting for the user to submit their guess. The search could already be ongoing while the user decides on a guess.

The path-finder could also be made multi-threaded (potentially in another language) and use a more efficient Queue implementation to increase search speed.
