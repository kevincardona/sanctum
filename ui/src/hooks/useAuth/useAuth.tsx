import { useMutation, useQuery, useQueryClient } from 'react-query';
import * as api from '../../features/auth/api';
import { SignInCredentials, SignInResponse } from '../../features/auth/types';

export default () => {
    const queryClient = useQueryClient();

    const signInMutation = useMutation<SignInResponse, unknown, SignInCredentials>(
        ({ username, password }) => api.signIn(username, password),
        {
            onSuccess: (data) => {
                localStorage.setItem('userToken', data.token);
                queryClient.invalidateQueries('user');
            },
        }
    );

    const signOutMutation = useMutation(api.signOut, {
        onSuccess: () => {
            queryClient.invalidateQueries('user');
        },
    });

    const { data: user, isLoading, isError } = useQuery(
        'user',
        api.getCurrentUser,
        {
            enabled: !!localStorage.getItem('userToken'),
        }
    );

    return {
        user,
        isLoading,
        isError,
        signIn: signInMutation.mutate,
        signOut: signOutMutation.mutate,
    };
}

