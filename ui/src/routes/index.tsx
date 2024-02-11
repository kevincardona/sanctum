import { useRoutes } from 'react-router-dom';
import Home from './Home';
import Login from './Login';

const Routes = () => {
  let routes = useRoutes([
    { path: '/home', element: <Home /> },
    { path: '/login', element: <Login /> },
  ]);

  return routes;
};

export default Routes;
