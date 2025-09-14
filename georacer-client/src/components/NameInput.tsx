import { useName } from '../contexts/NameContext';

const NameInput = () => {
  const { name, setName } = useName();

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  return (
    <div>
      <label htmlFor="name">Name:</label>
      <input
        type="text"
        id="name"
        value={name}
        onChange={handleChange}
        placeholder="Enter your name"
      />
    </div>
  );
};

export default NameInput;
