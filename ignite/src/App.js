import React, { useEffect } from "react";
//import components
import Home from "./pages/Home";
//style
import GlobalStyle from "./components/GlobalStyle";
import { Route } from "react-router-dom";

function App() {
  return (
    <div className="App">
      <GlobalStyle />
      <Route path={["/game/:id", "/"]}>
        <Home />
      </Route>
    </div>
  );
}

export default App;
