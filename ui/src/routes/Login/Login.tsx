import { useState } from 'react';
import useAuth from '../../hooks/useAuth';
import './Login.scss';

const Login = () => {
  const { signIn } = useAuth();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleLogin = (e: any) => {
    e.preventDefault();
    signIn({ email, password })
  }

  return (
    <div className="container--rows">
      <img className="logo--centered" src={process.env.PUBLIC_URL + '/images/logo.jpg'} alt="Logo" />
      <h3>Login</h3>
      <input type="text" placeholder="email" value={email} onChange={(e) => setEmail(e.target.value)} />
      <input type="password" placeholder="password" value={password} onChange={(e) => setPassword(e.target.value)} />
      <button onClick={handleLogin}>Login</button>
    </div>
  )

}

export default Login;

