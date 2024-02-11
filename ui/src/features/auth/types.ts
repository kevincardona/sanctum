export interface User {
    id: string,
    email: string,
    password: string,
}

export interface SignInResponse {
    user: User,
    token: string,
}

export interface AuthApiError {
    message: string,
}

export interface SignInCredentials {
    email: string;
    password: string;
}

