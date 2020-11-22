import React, {createContext, ReactNode, useState} from 'react';
import Cookies from 'js-cookie';

type UserContextProps = {
    isLoggedIn: boolean;
    setIsLoggedIn: (isLoggedIn: boolean) => void;
};

export const UserContext = createContext<UserContextProps>({} as any);

const UserContextProvider = ({children}: {children: ReactNode}) => {
    const hasCookie = !!Cookies.get("finch-sid");
    const [isLoggedIn, setIsLoggedIn] = useState<boolean>(hasCookie);

    return <UserContext.Provider value={{isLoggedIn, setIsLoggedIn}}>
        {children}
    </UserContext.Provider>
}

export default UserContextProvider;
