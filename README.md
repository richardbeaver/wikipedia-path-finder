# Where's Kevin? - A Wikipedia Path Finder

- Web-hosted game that displays a randomly chosen Wikipedia article and asks user
  to guess minimum number of link hops to reach Kevin Bacon’s Wikipedia page
  (inspired by the [Six Degrees of Kevin Bacon](https://en.wikipedia.org/wiki/Six_Degrees_of_Kevin_Bacon)).

- Uses Breadth-First Search to find the shortest path from the starting page to Kevin Bacon's Wikipedia page.

The user experience could be made more efficient by starting the path-finding when the page is loaded, rather than waiting for the user to submit their guess. The search could already be ongoing while the user decides on a guess.

The path-finder could also be made multi-threaded (potentially in another language) and use a more efficient Queue implementation to increase search speed.

### Required Environment Variables:

- VITE_FRONTEND_PORT
- VITE_BACKEND_PORT
- VITE_BACKEND_URL
