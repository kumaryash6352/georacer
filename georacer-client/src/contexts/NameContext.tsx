import { createContext, useContext, useState, ReactNode } from 'react';

interface NameContextType {
  name: string;
  setName: (name: string) => void;
}

const NameContext = createContext<NameContextType | undefined>(undefined);

const colors = ['red', 'orange', 'yellow', 'green', 'blue', 'indigo', 'violet'];

export const NameProvider = ({ children }: { children: ReactNode }) => {
  const [name, setName] = useState<string>(colors[Math.floor(Math.random() * colors.length)] + Math.floor(Math.random() * 100));

  return (
    <NameContext.Provider value={{ name, setName }}>
      {children}
    </NameContext.Provider>
  );
};

export const useName = () => {
  const context = useContext(NameContext);
  if (!context) {
    throw new Error('useName must be used within a NameProvider');
  }
  return context;
};
