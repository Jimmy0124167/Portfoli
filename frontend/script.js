const form = document.getElementById("contact-form");
const responseMsg = document.getElementById("response");

form.addEventListener("submit", async (e) => {
  e.preventDefault();

  const data = {
    name: form.name?.value || document.getElementById("name").value,
    email: form.email?.value || document.getElementById("email").value,
    message: document.getElementById("message").value,
  };

  try {
    const res = await fetch("/api/contact", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(data),
    });

    const result = await res.json();
    responseMsg.textContent = result.message || "Message sent!";
    form.reset();
  } catch (error) {
    responseMsg.textContent = "Something went wrong. Please try again later.";
  }
});
