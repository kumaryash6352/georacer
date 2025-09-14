import { useName } from '../contexts/NameContext';

const NameInput = () => {
  const { name, setName } = useName();

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  return (
    <div className="ui-stack">
      <label htmlFor="name">Name</label>
      <input id="name" className="ui-input" value={name} onChange={handleChange} placeholder="Enter your name" />
    </div>
  );
};

export default NameInput;
