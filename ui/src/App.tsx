import './App.scss';
import React from 'react';
import { BrowserRouter as Router } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from 'react-query';
import Routes from './routes';

const queryClient: QueryClient = new QueryClient();

function App(): React.ReactElement | null {
  return (
    <div className="app">
      <QueryClientProvider client={queryClient}>
        <Router>
          <Routes />
        </Router>
      </QueryClientProvider>
    </div>
  );
}

export default App;
