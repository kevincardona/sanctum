import { User, SignInResponse, AuthApiError } from './types';

const signIn = async (username: string, password: string): Promise<SignInResponse> => {
    try {
        const response = await fetch('/api/v1/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password }),
        });
        if (!response.ok) {
            const error: AuthApiError = await response.json();
            throw new Error(error.message);
        }
        const data: SignInResponse = await response.json();
        return data;
    } catch (error) {
        throw new Error('Sign in failed');
    }
};

const signOut = async () => {
    const response = await fetch('/api/auth/signout', { method: 'POST' });

    if (!response.ok) {
        throw new Error('Sign out failed');
    }

    return response.json();
};

const getCurrentUser = async () => {
    const response = await fetch('/api/auth/session');

    if (!response.ok) {
        throw new Error('Fetching current user failed');
    }

    return response.json();
};

export { signIn, signOut, getCurrentUser };
