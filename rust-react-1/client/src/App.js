import logo from "./logo.svg";
import "./App.css";
import { useEffect } from "react";

function App() {
  useEffect(() => {
    const fn = async () => {
      // Set your parameter values
      const height = 100;
      const width = 200;
      const cx = 50;
      const cy = 75;

      // Create URLSearchParams object to handle parameters
      const params = new URLSearchParams();
      params.append("height", height);
      params.append("width", width);
      params.append("cx", cx);
      params.append("cy", cy);

      // Append parameters to the URL
      let url = `http://localhost:8080/get-image?${params.toString()}`;

      // Make the fetch request
      let response = await fetch(url);

      // Parse and log the JSON response
      let data = await response.json();
      console.log(data);
    };

    // Call the function
    fn();
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
