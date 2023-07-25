import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";

import {createBrowserRouter, RouterProvider} from 'react-router-dom';
import {Outlet} from 'react-router-dom'

import ErrorPage from './routes/ErrorPage'

import App from './routes/App';
import Contact from './routes/Contact';

const router = createBrowserRouter([
  {
    path: "/",
    errorElement: <ErrorPage />,
    element: <App />
  },
  {
    path: "/contact",
    element: <Contact />
  },
])


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router}/>

  </React.StrictMode>,
);
