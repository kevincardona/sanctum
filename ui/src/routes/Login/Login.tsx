import { useState } from 'react';
import useAuth from '../../hooks/useAuth';
import './Login.scss';

const Login = () => {
  const { signIn } = useAuth();
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  const handleLogin = (e: any) => {
    e.preventDefault();
    signIn({ username, password })
  }

  return (
    <div className="container--rows">
      <img className="logo--centered" src={process.env.PUBLIC_URL + '/images/logo.jpg'} alt="Logo" />
      <h3>Login</h3>
      <input type="text" placeholder="username" value={username} onChange={(e) => setUsername(e.target.value)} />
      <input type="password" placeholder="password" value={password} onChange={(e) => setPassword(e.target.value)} />
      <button onClick={handleLogin}>Login</button>
    </div>
  )

}

export default Login;

