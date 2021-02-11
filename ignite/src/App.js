import React, { useEffect } from "react";
//import components
import Home from "./pages/Home";
import Nav from "./components/Nav";
//style
import GlobalStyle from "./components/GlobalStyle";
//router
import { Route } from "react-router-dom";


function App() {
  return (
    <div className="App">
      <GlobalStyle />
      <Nav />
      <Route path={["/game/:id", "/"]}>
        <Home />
      </Route>
    </div>
  );
}

export default App;
