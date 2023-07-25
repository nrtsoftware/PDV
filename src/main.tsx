import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";

import {createBrowserRouter, RouterProvider} from 'react-router-dom';
// import {Outlet} from 'react-router-dom'

import ErrorPage from './routes/ErrorPage'
import App from './App'
import Dashboard from './routes/Dashboard'
import Vendas from "./routes/vendas/Index";
import Compras from "./routes/compras/Index";
import Configs from "./routes/configs";
import Vender from "./routes/vendas/Vender";
import AlterarVenda from "./routes/vendas/AlterarVenda";
import CancelarVenda from "./routes/vendas/CancelarVenda";
import AlterarCompra from "./routes/compras/AlterarCompra";
import CancelarCompra from "./routes/compras/CancelarCompra";
import RegistrarCompra from "./routes/compras/RegistrarCompra";


const router = createBrowserRouter([
  {
    path: "/",
    errorElement: <ErrorPage />,
    element: <App />
  },
  {
    path: "/dashboard",
    errorElement: <ErrorPage />,
    element: <Dashboard />,
  },
  {
    path: "/vendas",
    element: <Vendas />,
    children: [
      {
        path: "/vendas/vender",
        element: <Vender />
      },
      {
        path: "/vendas/alterar",
        element: <AlterarVenda />
      },
      {
        path: "/vendas/cancelar",
        element: <CancelarVenda />
      }
    ]
  },
  {
    path: "/compras",
    element: <Compras />,
    children: [
      {
        path: "/compras/registrar",
        element: <RegistrarCompra />
      },
      {
        path: "/compras/alterar",
        element: <AlterarCompra />
      },
      {
        path: "/compras/cancelar",
        element: <CancelarCompra />
      },
    ]
  },
  {
    path: "/configs",
    element: <Configs />,
    children: [
    //   {
    //     path: "/Configs/Users",
    //     element: <Users />,
    //     children:[
    //       {
    //         path: "/Configs/Vendedores",
    //         element: <Vendedores />,
    //       },
    //       {
    //         path: "/Configs/Gestores",
    //         element: <Gestores />,
    //       },
    //       {
    //         path: "/Configs/Admins",
    //         element: <Admins />,
    //       }
    //     ]
        
    //   },
    //   {
    //     path: "/Configs/Alterar",
    //     element: <Alterar />
    //   },
    //   {
    //     path: "/Configs/Cancelar",
    //     element: <Cancelar />
    //   },
    ]
  },

])


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router}/>
  </React.StrictMode>,
);
