class Badge {
    public String print(Integer id, String name, String department) {
        StringBuilder result = new StringBuilder();

        if (id != null) {
            result.append("[").append(id).append("] - ");
        }

        result.append(name).append(" - ");

        if (department == null) {
            result.append("OWNER");
        } else {
            result.append(department.toUpperCase());
        }

        return result.toString();
    }
}
