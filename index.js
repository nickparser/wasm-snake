const rust = import('./pkg');

const newGame = () => {
    rust
        .then(m => m.game("game", 50, 50, 15, 20))
        .catch(console.error);
};

newGame();