import {Link} from 'react-router-dom'

const Navbar = () => {
  return (

    <div>
      <p>
        <Link to='/'>DashBoard</Link>
      </p>
      <p>
        <Link to='/vendas'>Vendas</Link>
      </p>
      <p>
        <Link to='/compras'>Compras</Link>
      </p>
      <p>
        <Link to='/configs'>Configs</Link>
      </p>
    </div>
  )
}

export default Navbar