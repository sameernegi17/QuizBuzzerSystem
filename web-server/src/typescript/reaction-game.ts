document.addEventListener('DOMContentLoaded', function () {
  const startBtn = document.getElementById('start-btn');
  const counter = document.getElementById('counter');
  const signal = document.getElementById('signal');

  startBtn.addEventListener('click', startGame);

  function startGame() {
    startBtn.disabled = true;
    counter.classList.remove('d-none');
    countdown(3, go);
  }

  function countdown(time, callback) {
    counter.innerText = time;
    const countdownInterval = setInterval(() => {
      time--;
      counter.innerText = time;
      if (time === 0) {
        clearInterval(countdownInterval);
        callback();
      }
    }, 1000);
  }

  function go() {
    signal.classList.remove('d-none');
    const randomDelay = Math.floor(Math.random() * 5000);
    setTimeout(() => {
      signal.innerText = 'React now!';
      signal.classList.add('bg-danger', 'text-white');
      document.addEventListener('click', reactionTime);
    }, randomDelay);
  }

  function reactionTime() {
    const reactionTime = Date.now();
    document.removeEventListener('click', reactionTime);
    signal.innerText = `Reaction time: ${reactionTime}ms`;
    signal.classList.remove('bg-danger', 'text-white');
    startBtn.disabled = false;
  }
});
