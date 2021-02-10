import React, { useEffect } from "react";
//import components
import Home from "./pages/Home";
//style
import GlobalStyle from "./components/GlobalStyle";

function App() {

  return (
    <div className="App">
      <GlobalStyle />
      <Home />
    </div>
  );
}

export default App;
