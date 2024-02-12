export interface User {
    id: string,
    username: string,
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
    username: string;
    password: string;
}

