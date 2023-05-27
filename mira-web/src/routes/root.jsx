import React from "react";
import NavBar from "../components/NavBar.js";
import { Outlet } from 'react-router-dom';

export default function Root() {
  return (
    <div className="App">
      <NavBar>
        <Outlet />
      </NavBar>
    </div>
  );
}
